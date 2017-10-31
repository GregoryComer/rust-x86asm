use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmpsadbw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMPSADBW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM2)), operand4: Some(Literal8(32)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 65, 66, 218, 32], OperandSize::Dword)
}

#[test]
fn vmpsadbw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMPSADBW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, ECX, Four, 735139479, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(27)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 113, 66, 148, 138, 151, 86, 209, 43, 27], OperandSize::Dword)
}

#[test]
fn vmpsadbw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMPSADBW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM1)), operand4: Some(Literal8(69)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 105, 66, 217, 69], OperandSize::Qword)
}

#[test]
fn vmpsadbw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMPSADBW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(RDI, RDX, Eight, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(98)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 66, 44, 215, 98], OperandSize::Qword)
}

#[test]
fn vmpsadbw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMPSADBW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM3)), operand4: Some(Literal8(15)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 109, 66, 251, 15], OperandSize::Dword)
}

#[test]
fn vmpsadbw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMPSADBW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM0)), operand3: Some(Indirect(EBX, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(92)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 66, 59, 92], OperandSize::Dword)
}

#[test]
fn vmpsadbw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMPSADBW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM5)), operand4: Some(Literal8(113)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 77, 66, 237, 113], OperandSize::Qword)
}

#[test]
fn vmpsadbw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMPSADBW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(RBX, RBX, Two, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(110)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 66, 44, 91, 110], OperandSize::Qword)
}

