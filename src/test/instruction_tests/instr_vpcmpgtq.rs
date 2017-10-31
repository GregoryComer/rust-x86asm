use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpcmpgtq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 55, 248], OperandSize::Dword)
}

#[test]
fn vpcmpgtq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(EAX, 284758965, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 55, 176, 181, 19, 249, 16], OperandSize::Dword)
}

#[test]
fn vpcmpgtq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 55, 248], OperandSize::Qword)
}

#[test]
fn vpcmpgtq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 55, 41], OperandSize::Qword)
}

#[test]
fn vpcmpgtq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 93, 55, 249], OperandSize::Dword)
}

#[test]
fn vpcmpgtq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM6)), operand3: Some(Indirect(EDX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 55, 50], OperandSize::Dword)
}

#[test]
fn vpcmpgtq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 93, 55, 249], OperandSize::Qword)
}

#[test]
fn vpcmpgtq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: Some(Indirect(RBX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 55, 11], OperandSize::Qword)
}

#[test]
fn vpcmpgtq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(K5)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 245, 14, 55, 238], OperandSize::Dword)
}

#[test]
fn vpcmpgtq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectDisplaced(EDI, 405700626, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 213, 12, 55, 143, 18, 128, 46, 24], OperandSize::Dword)
}

#[test]
fn vpcmpgtq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(K2)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(EDI, 1965041856, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 205, 28, 55, 151, 192, 40, 32, 117], OperandSize::Dword)
}

#[test]
fn vpcmpgtq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(K5)), operand2: Some(Direct(XMM14)), operand3: Some(Direct(XMM21)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 178, 141, 12, 55, 237], OperandSize::Qword)
}

#[test]
fn vpcmpgtq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(K5)), operand2: Some(Direct(XMM23)), operand3: Some(IndirectDisplaced(RCX, 66300852, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 197, 5, 55, 169, 180, 171, 243, 3], OperandSize::Qword)
}

#[test]
fn vpcmpgtq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(K6)), operand2: Some(Direct(XMM11)), operand3: Some(IndirectScaledIndexed(RAX, RSI, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 165, 25, 55, 52, 240], OperandSize::Qword)
}

#[test]
fn vpcmpgtq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(K1)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 205, 43, 55, 207], OperandSize::Dword)
}

#[test]
fn vpcmpgtq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(K4)), operand2: Some(Direct(YMM3)), operand3: Some(Indirect(ESI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 229, 42, 55, 38], OperandSize::Dword)
}

#[test]
fn vpcmpgtq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(K5)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, ESI, Four, 991965723, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 197, 63, 55, 172, 182, 27, 50, 32, 59], OperandSize::Dword)
}

#[test]
fn vpcmpgtq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(K5)), operand2: Some(Direct(YMM11)), operand3: Some(Direct(YMM28)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 146, 165, 41, 55, 236], OperandSize::Qword)
}

#[test]
fn vpcmpgtq_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(K4)), operand2: Some(Direct(YMM31)), operand3: Some(Indirect(RSI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 133, 36, 55, 38], OperandSize::Qword)
}

#[test]
fn vpcmpgtq_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(K5)), operand2: Some(Direct(YMM24)), operand3: Some(IndirectScaledDisplaced(RAX, Eight, 1252229512, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 189, 49, 55, 44, 197, 136, 129, 163, 74], OperandSize::Qword)
}

#[test]
fn vpcmpgtq_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(K5)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 205, 75, 55, 237], OperandSize::Dword)
}

#[test]
fn vpcmpgtq_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(K7)), operand2: Some(Direct(ZMM7)), operand3: Some(Indirect(EDI, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 197, 75, 55, 63], OperandSize::Dword)
}

#[test]
fn vpcmpgtq_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(K3)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectDisplaced(ESI, 1742249973, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 221, 91, 55, 158, 245, 159, 216, 103], OperandSize::Dword)
}

#[test]
fn vpcmpgtq_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(K7)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 221, 78, 55, 253], OperandSize::Qword)
}

#[test]
fn vpcmpgtq_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(K3)), operand2: Some(Direct(ZMM18)), operand3: Some(IndirectScaledIndexed(RDI, RCX, Eight, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 237, 68, 55, 28, 207], OperandSize::Qword)
}

#[test]
fn vpcmpgtq_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(K5)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectDisplaced(RBX, 162566593, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 205, 95, 55, 171, 193, 145, 176, 9], OperandSize::Qword)
}

