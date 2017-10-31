use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpcmpgtq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 55, 249], OperandSize::Dword)
}

#[test]
fn vpcmpgtq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(EDX, EAX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 55, 12, 66], OperandSize::Dword)
}

#[test]
fn vpcmpgtq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 55, 241], OperandSize::Qword)
}

#[test]
fn vpcmpgtq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(RSI, Two, 1248500213, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 55, 36, 117, 245, 153, 106, 74], OperandSize::Qword)
}

#[test]
fn vpcmpgtq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 55, 247], OperandSize::Dword)
}

#[test]
fn vpcmpgtq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(Indirect(ECX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 55, 33], OperandSize::Dword)
}

#[test]
fn vpcmpgtq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 55, 197], OperandSize::Qword)
}

#[test]
fn vpcmpgtq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectDisplaced(RAX, 610798315, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 93, 55, 144, 235, 10, 104, 36], OperandSize::Qword)
}

#[test]
fn vpcmpgtq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(K5)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 229, 13, 55, 233], OperandSize::Dword)
}

#[test]
fn vpcmpgtq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(K2)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(ECX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 205, 9, 55, 17], OperandSize::Dword)
}

#[test]
fn vpcmpgtq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(K4)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(EDI, Eight, 844133478, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 221, 29, 55, 36, 253, 102, 116, 80, 50], OperandSize::Dword)
}

#[test]
fn vpcmpgtq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(K2)), operand2: Some(Direct(XMM30)), operand3: Some(Direct(XMM21)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 178, 141, 2, 55, 213], OperandSize::Qword)
}

#[test]
fn vpcmpgtq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(K4)), operand2: Some(Direct(XMM10)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RCX, Four, 971735266, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 173, 12, 55, 164, 136, 226, 128, 235, 57], OperandSize::Qword)
}

#[test]
fn vpcmpgtq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(K2)), operand2: Some(Direct(XMM16)), operand3: Some(IndirectDisplaced(RCX, 1298536433, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 253, 17, 55, 145, 241, 23, 102, 77], OperandSize::Qword)
}

#[test]
fn vpcmpgtq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 237, 43, 55, 211], OperandSize::Dword)
}

#[test]
fn vpcmpgtq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(K7)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EDI, Two, 1719927930, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 237, 47, 55, 188, 123, 122, 4, 132, 102], OperandSize::Dword)
}

#[test]
fn vpcmpgtq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(K1)), operand2: Some(Direct(YMM3)), operand3: Some(Indirect(EDI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 229, 61, 55, 15], OperandSize::Dword)
}

#[test]
fn vpcmpgtq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(K1)), operand2: Some(Direct(YMM30)), operand3: Some(Direct(YMM19)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 178, 141, 33, 55, 203], OperandSize::Qword)
}

#[test]
fn vpcmpgtq_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(K7)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectDisplaced(RDX, 462181187, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 253, 42, 55, 186, 67, 83, 140, 27], OperandSize::Qword)
}

#[test]
fn vpcmpgtq_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(K7)), operand2: Some(Direct(YMM27)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RBX, Two, 2038925517, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 165, 55, 55, 188, 90, 205, 136, 135, 121], OperandSize::Qword)
}

#[test]
fn vpcmpgtq_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(K3)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 229, 74, 55, 221], OperandSize::Dword)
}

#[test]
fn vpcmpgtq_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(K5)), operand2: Some(Direct(ZMM7)), operand3: Some(Indirect(EDX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 197, 74, 55, 42], OperandSize::Dword)
}

#[test]
fn vpcmpgtq_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(K6)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledIndexed(EAX, EBX, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 221, 92, 55, 52, 152], OperandSize::Dword)
}

#[test]
fn vpcmpgtq_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(K5)), operand2: Some(Direct(ZMM27)), operand3: Some(Direct(ZMM31)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 146, 165, 66, 55, 239], OperandSize::Qword)
}

#[test]
fn vpcmpgtq_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(K3)), operand2: Some(Direct(ZMM26)), operand3: Some(IndirectScaledDisplaced(RSI, Eight, 340260635, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 173, 69, 55, 28, 245, 27, 247, 71, 20], OperandSize::Qword)
}

#[test]
fn vpcmpgtq_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(K7)), operand2: Some(Direct(ZMM17)), operand3: Some(IndirectScaledDisplaced(RCX, Eight, 1433234377, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 245, 82, 55, 60, 205, 201, 107, 109, 85], OperandSize::Qword)
}

