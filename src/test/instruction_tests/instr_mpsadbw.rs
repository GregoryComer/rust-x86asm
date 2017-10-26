use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn mpsadbw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MPSADBW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(Literal8(72)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 66, 234, 72], OperandSize::Dword)
}

#[test]
fn mpsadbw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MPSADBW, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, EDI, Eight, 1412603938, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(90)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 66, 164, 249, 34, 160, 50, 84, 90], OperandSize::Dword)
}

#[test]
fn mpsadbw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MPSADBW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(Literal8(118)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 66, 231, 118], OperandSize::Qword)
}

#[test]
fn mpsadbw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MPSADBW, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledDisplaced(RBX, Two, 2138449010, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(45)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 66, 52, 93, 114, 36, 118, 127, 45], OperandSize::Qword)
}

