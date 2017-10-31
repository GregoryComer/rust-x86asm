use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vprorq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(Literal8(19)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 253, 140, 114, 198, 19], OperandSize::Dword)
}

#[test]
fn vprorq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORQ, operand1: Some(Direct(XMM0)), operand2: Some(IndirectDisplaced(EBX, 712247857, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(125)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 253, 138, 114, 131, 49, 10, 116, 42, 125], OperandSize::Dword)
}

#[test]
fn vprorq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORQ, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, EDI, Two, 1294285172, Some(OperandSize::Qword), None)), operand3: Some(Literal8(98)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 237, 158, 114, 132, 122, 116, 57, 37, 77, 98], OperandSize::Dword)
}

#[test]
fn vprorq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORQ, operand1: Some(Direct(XMM12)), operand2: Some(Direct(XMM3)), operand3: Some(Literal8(14)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 157, 138, 114, 195, 14], OperandSize::Qword)
}

#[test]
fn vprorq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORQ, operand1: Some(Direct(XMM22)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RBX, Four, 613858941, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(27)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 205, 135, 114, 132, 158, 125, 190, 150, 36, 27], OperandSize::Qword)
}

#[test]
fn vprorq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORQ, operand1: Some(Direct(XMM10)), operand2: Some(IndirectScaledDisplaced(RDI, Four, 964477089, Some(OperandSize::Qword), None)), operand3: Some(Literal8(3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 173, 158, 114, 4, 189, 161, 192, 124, 57, 3], OperandSize::Qword)
}

#[test]
fn vprorq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM0)), operand3: Some(Literal8(111)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 213, 175, 114, 192, 111], OperandSize::Dword)
}

#[test]
fn vprorq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORQ, operand1: Some(Direct(YMM7)), operand2: Some(Indirect(EBX, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(112)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 197, 171, 114, 3, 112], OperandSize::Dword)
}

#[test]
fn vprorq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORQ, operand1: Some(Direct(YMM1)), operand2: Some(Indirect(EDX, Some(OperandSize::Qword), None)), operand3: Some(Literal8(27)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 245, 189, 114, 2, 27], OperandSize::Dword)
}

#[test]
fn vprorq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORQ, operand1: Some(Direct(YMM15)), operand2: Some(Direct(YMM7)), operand3: Some(Literal8(44)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 133, 169, 114, 199, 44], OperandSize::Qword)
}

#[test]
fn vprorq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORQ, operand1: Some(Direct(YMM15)), operand2: Some(IndirectScaledIndexed(RDX, RCX, Four, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(27)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 133, 171, 114, 4, 138, 27], OperandSize::Qword)
}

#[test]
fn vprorq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORQ, operand1: Some(Direct(YMM14)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RDX, Two, 1700860399, Some(OperandSize::Qword), None)), operand3: Some(Literal8(3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 141, 190, 114, 132, 83, 239, 17, 97, 101, 3], OperandSize::Qword)
}

#[test]
fn vprorq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORQ, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM2)), operand3: Some(Literal8(26)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 221, 204, 114, 194, 26], OperandSize::Dword)
}

#[test]
fn vprorq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORQ, operand1: Some(Direct(ZMM4)), operand2: Some(IndirectDisplaced(EDX, 1506023208, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(56)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 221, 207, 114, 130, 40, 23, 196, 89, 56], OperandSize::Dword)
}

#[test]
fn vprorq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORQ, operand1: Some(Direct(ZMM7)), operand2: Some(IndirectDisplaced(EDI, 1739411598, Some(OperandSize::Qword), None)), operand3: Some(Literal8(36)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 197, 223, 114, 135, 142, 80, 173, 103, 36], OperandSize::Dword)
}

#[test]
fn vprorq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORQ, operand1: Some(Direct(ZMM15)), operand2: Some(Direct(ZMM18)), operand3: Some(Literal8(64)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 177, 133, 201, 114, 194, 64], OperandSize::Qword)
}

#[test]
fn vprorq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORQ, operand1: Some(Direct(ZMM17)), operand2: Some(IndirectDisplaced(RDX, 1518561679, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(13)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 245, 195, 114, 130, 143, 105, 131, 90, 13], OperandSize::Qword)
}

#[test]
fn vprorq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORQ, operand1: Some(Direct(ZMM27)), operand2: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand3: Some(Literal8(39)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 165, 213, 114, 7, 39], OperandSize::Qword)
}

