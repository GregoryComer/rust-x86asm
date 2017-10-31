use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn mpsadbw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MPSADBW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(Literal8(9)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 66, 249, 9], OperandSize::Dword)
}

#[test]
fn mpsadbw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MPSADBW, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(EAX, Eight, 482851242, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(118)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 66, 36, 197, 170, 185, 199, 28, 118], OperandSize::Dword)
}

#[test]
fn mpsadbw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MPSADBW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(Literal8(124)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 66, 219, 124], OperandSize::Qword)
}

#[test]
fn mpsadbw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MPSADBW, operand1: Some(Direct(XMM5)), operand2: Some(IndirectDisplaced(RDI, 2061049436, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(15)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 66, 175, 92, 30, 217, 122, 15], OperandSize::Qword)
}

