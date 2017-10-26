use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsravq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 221, 142, 70, 225], OperandSize::Dword)
}

#[test]
fn vpsravq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(ECX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 237, 141, 70, 57], OperandSize::Dword)
}

#[test]
fn vpsravq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(ECX, EBX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 197, 157, 70, 28, 217], OperandSize::Dword)
}

#[test]
fn vpsravq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM22)), operand3: Some(Direct(XMM30)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 146, 205, 132, 70, 214], OperandSize::Qword)
}

#[test]
fn vpsravq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVQ, operand1: Some(Direct(XMM30)), operand2: Some(Direct(XMM26)), operand3: Some(IndirectScaledIndexed(RCX, RDI, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 98, 173, 132, 70, 52, 121], OperandSize::Qword)
}

#[test]
fn vpsravq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVQ, operand1: Some(Direct(XMM9)), operand2: Some(Direct(XMM12)), operand3: Some(Indirect(RSI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 114, 157, 155, 70, 14], OperandSize::Qword)
}

#[test]
fn vpsravq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 253, 175, 70, 198], OperandSize::Dword)
}

#[test]
fn vpsravq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM0)), operand3: Some(Indirect(EAX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 253, 171, 70, 40], OperandSize::Dword)
}

#[test]
fn vpsravq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexed(ESI, ECX, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 221, 188, 70, 44, 142], OperandSize::Dword)
}

#[test]
fn vpsravq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVQ, operand1: Some(Direct(YMM15)), operand2: Some(Direct(YMM22)), operand3: Some(Direct(YMM11)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 82, 205, 163, 70, 251], OperandSize::Qword)
}

#[test]
fn vpsravq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVQ, operand1: Some(Direct(YMM15)), operand2: Some(Direct(YMM0)), operand3: Some(Indirect(RAX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 114, 253, 171, 70, 56], OperandSize::Qword)
}

#[test]
fn vpsravq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVQ, operand1: Some(Direct(YMM20)), operand2: Some(Direct(YMM29)), operand3: Some(IndirectDisplaced(RSI, 878006522, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 226, 149, 177, 70, 166, 250, 80, 85, 52], OperandSize::Qword)
}

#[test]
fn vpsravq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVQ, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 205, 203, 70, 217], OperandSize::Dword)
}

#[test]
fn vpsravq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVQ, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledDisplaced(EBX, Two, 2031872757, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 229, 207, 70, 52, 93, 245, 234, 27, 121], OperandSize::Dword)
}

#[test]
fn vpsravq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVQ, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EDI, Eight, 1474711502, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 213, 217, 70, 156, 255, 206, 79, 230, 87], OperandSize::Dword)
}

#[test]
fn vpsravq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVQ, operand1: Some(Direct(ZMM27)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 98, 213, 203, 70, 222], OperandSize::Qword)
}

#[test]
fn vpsravq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVQ, operand1: Some(Direct(ZMM17)), operand2: Some(Direct(ZMM7)), operand3: Some(Indirect(RAX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 226, 197, 201, 70, 8], OperandSize::Qword)
}

#[test]
fn vpsravq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVQ, operand1: Some(Direct(ZMM8)), operand2: Some(Direct(ZMM14)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RSI, Two, 744913242, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 141, 219, 70, 132, 115, 90, 121, 102, 44], OperandSize::Qword)
}

