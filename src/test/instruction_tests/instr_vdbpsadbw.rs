use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vdbpsadbw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VDBPSADBW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM2)), operand4: Some(Literal8(93)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 69, 142, 66, 202, 93], OperandSize::Dword)
}

#[test]
fn vdbpsadbw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VDBPSADBW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(EDI, Four, 1875484248, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(97)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 101, 140, 66, 4, 189, 88, 158, 201, 111, 97], OperandSize::Dword)
}

#[test]
fn vdbpsadbw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VDBPSADBW, operand1: Some(Direct(XMM13)), operand2: Some(Direct(XMM21)), operand3: Some(Direct(XMM21)), operand4: Some(Literal8(109)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 51, 85, 132, 66, 237, 109], OperandSize::Qword)
}

#[test]
fn vdbpsadbw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VDBPSADBW, operand1: Some(Direct(XMM22)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledDisplaced(RCX, Four, 341913354, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(3)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 227, 77, 139, 66, 52, 141, 10, 47, 97, 20, 3], OperandSize::Qword)
}

#[test]
fn vdbpsadbw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VDBPSADBW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM0)), operand4: Some(Literal8(89)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 125, 175, 66, 248, 89], OperandSize::Dword)
}

#[test]
fn vdbpsadbw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VDBPSADBW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledDisplaced(ESI, Four, 1441455171, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(82)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 125, 170, 66, 52, 181, 67, 220, 234, 85, 82], OperandSize::Dword)
}

#[test]
fn vdbpsadbw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VDBPSADBW, operand1: Some(Direct(YMM13)), operand2: Some(Direct(YMM29)), operand3: Some(Direct(YMM22)), operand4: Some(Literal8(51)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 51, 21, 162, 66, 238, 51], OperandSize::Qword)
}

#[test]
fn vdbpsadbw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VDBPSADBW, operand1: Some(Direct(YMM28)), operand2: Some(Direct(YMM11)), operand3: Some(IndirectScaledIndexed(RDX, RDX, Eight, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(93)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 99, 37, 173, 66, 36, 210, 93], OperandSize::Qword)
}

#[test]
fn vdbpsadbw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VDBPSADBW, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM4)), operand4: Some(Literal8(36)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 117, 203, 66, 204, 36], OperandSize::Dword)
}

#[test]
fn vdbpsadbw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VDBPSADBW, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledIndexed(ESI, EAX, Eight, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(73)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 125, 207, 66, 20, 198, 73], OperandSize::Dword)
}

#[test]
fn vdbpsadbw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VDBPSADBW, operand1: Some(Direct(ZMM26)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM27)), operand4: Some(Literal8(123)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 3, 69, 203, 66, 211, 123], OperandSize::Qword)
}

#[test]
fn vdbpsadbw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VDBPSADBW, operand1: Some(Direct(ZMM12)), operand2: Some(Direct(ZMM25)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RBX, Eight, 938432062, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(83)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 115, 53, 198, 66, 164, 218, 62, 86, 239, 55, 83], OperandSize::Qword)
}

