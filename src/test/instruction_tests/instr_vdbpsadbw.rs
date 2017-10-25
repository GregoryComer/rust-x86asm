use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vdbpsadbw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VDBPSADBW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM6)), operand4: Some(Literal8(110)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 109, 137, 66, 230, 110], OperandSize::Dword)
}

#[test]
fn vdbpsadbw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VDBPSADBW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(EDI, 1427264392, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(17)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 101, 142, 66, 159, 136, 83, 18, 85, 17], OperandSize::Dword)
}

#[test]
fn vdbpsadbw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VDBPSADBW, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM24)), operand3: Some(Direct(XMM11)), operand4: Some(Literal8(6)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 67, 61, 129, 66, 251, 6], OperandSize::Qword)
}

#[test]
fn vdbpsadbw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VDBPSADBW, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RDX, Four, 776008285, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(113)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 115, 85, 139, 66, 188, 144, 93, 242, 64, 46, 113], OperandSize::Qword)
}

#[test]
fn vdbpsadbw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VDBPSADBW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM7)), operand4: Some(Literal8(69)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 125, 174, 66, 207, 69], OperandSize::Dword)
}

#[test]
fn vdbpsadbw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VDBPSADBW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: Some(Indirect(ECX, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(126)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 85, 172, 66, 17, 126], OperandSize::Dword)
}

#[test]
fn vdbpsadbw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VDBPSADBW, operand1: Some(Direct(YMM31)), operand2: Some(Direct(YMM11)), operand3: Some(Direct(YMM1)), operand4: Some(Literal8(52)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 99, 37, 170, 66, 249, 52], OperandSize::Qword)
}

#[test]
fn vdbpsadbw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VDBPSADBW, operand1: Some(Direct(YMM21)), operand2: Some(Direct(YMM8)), operand3: Some(Indirect(RDX, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(85)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 227, 61, 170, 66, 42, 85], OperandSize::Qword)
}

#[test]
fn vdbpsadbw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VDBPSADBW, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM5)), operand4: Some(Literal8(63)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 109, 203, 66, 205, 63], OperandSize::Dword)
}

#[test]
fn vdbpsadbw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VDBPSADBW, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EDI, Four, 830440103, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(20)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 125, 203, 66, 148, 186, 167, 130, 127, 49, 20], OperandSize::Dword)
}

#[test]
fn vdbpsadbw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VDBPSADBW, operand1: Some(Direct(ZMM13)), operand2: Some(Direct(ZMM15)), operand3: Some(Direct(ZMM4)), operand4: Some(Literal8(89)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 115, 5, 203, 66, 236, 89], OperandSize::Qword)
}

#[test]
fn vdbpsadbw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VDBPSADBW, operand1: Some(Direct(ZMM27)), operand2: Some(Direct(ZMM15)), operand3: Some(IndirectScaledIndexed(RSI, RSI, Two, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(3)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 99, 5, 205, 66, 28, 118, 3], OperandSize::Qword)
}

