use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsllvq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 233, 71, 209], OperandSize::Dword)
}

#[test]
fn vpsllvq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(ECX, 2092082575, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 225, 71, 129, 143, 165, 178, 124], OperandSize::Dword)
}

#[test]
fn vpsllvq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 241, 71, 235], OperandSize::Qword)
}

#[test]
fn vpsllvq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 233, 71, 51], OperandSize::Qword)
}

#[test]
fn vpsllvq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 229, 71, 229], OperandSize::Dword)
}

#[test]
fn vpsllvq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, ESI, Eight, 258690794, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 205, 71, 156, 243, 234, 78, 107, 15], OperandSize::Dword)
}

#[test]
fn vpsllvq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 237, 71, 218], OperandSize::Qword)
}

#[test]
fn vpsllvq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledDisplaced(RBX, Eight, 1343613531, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 197, 71, 44, 221, 91, 234, 21, 80], OperandSize::Qword)
}

#[test]
fn vpsllvq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 205, 141, 71, 242], OperandSize::Dword)
}

#[test]
fn vpsllvq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 197, 142, 71, 3], OperandSize::Dword)
}

#[test]
fn vpsllvq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(ESI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 221, 153, 71, 22], OperandSize::Dword)
}

#[test]
fn vpsllvq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM16)), operand3: Some(Direct(XMM12)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 210, 253, 132, 71, 244], OperandSize::Qword)
}

#[test]
fn vpsllvq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(XMM30)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RDI, Four, 202471030, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 98, 205, 141, 71, 180, 187, 118, 118, 17, 12], OperandSize::Qword)
}

#[test]
fn vpsllvq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM31)), operand3: Some(IndirectScaledIndexed(RDX, RBX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 133, 146, 71, 12, 90], OperandSize::Qword)
}

#[test]
fn vpsllvq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 197, 169, 71, 196], OperandSize::Dword)
}

#[test]
fn vpsllvq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EAX, Eight, 1907146453, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 213, 172, 71, 180, 192, 213, 190, 172, 113], OperandSize::Dword)
}

#[test]
fn vpsllvq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectDisplaced(EBX, 1586810377, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 205, 190, 71, 179, 9, 206, 148, 94], OperandSize::Dword)
}

#[test]
fn vpsllvq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(YMM19)), operand2: Some(Direct(YMM21)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 226, 213, 167, 71, 223], OperandSize::Qword)
}

#[test]
fn vpsllvq_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(YMM9)), operand2: Some(Direct(YMM28)), operand3: Some(IndirectScaledIndexed(RDX, RAX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 114, 157, 167, 71, 12, 130], OperandSize::Qword)
}

#[test]
fn vpsllvq_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(YMM27)), operand2: Some(Direct(YMM30)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RDX, Four, 132242520, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 98, 141, 180, 71, 156, 147, 88, 220, 225, 7], OperandSize::Qword)
}

#[test]
fn vpsllvq_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 213, 206, 71, 222], OperandSize::Dword)
}

#[test]
fn vpsllvq_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM1)), operand3: Some(Indirect(ECX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 245, 204, 71, 33], OperandSize::Dword)
}

#[test]
fn vpsllvq_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectDisplaced(EDI, 1353986585, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 245, 217, 71, 175, 25, 50, 180, 80], OperandSize::Dword)
}

#[test]
fn vpsllvq_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(ZMM10)), operand2: Some(Direct(ZMM21)), operand3: Some(Direct(ZMM26)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 18, 213, 198, 71, 210], OperandSize::Qword)
}

#[test]
fn vpsllvq_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM9)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RDI, Four, 2074674941, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 181, 203, 71, 172, 186, 253, 6, 169, 123], OperandSize::Qword)
}

#[test]
fn vpsllvq_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(ZMM23)), operand2: Some(Direct(ZMM11)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RDI, Eight, 1565530481, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 226, 165, 223, 71, 188, 254, 113, 25, 80, 93], OperandSize::Qword)
}

