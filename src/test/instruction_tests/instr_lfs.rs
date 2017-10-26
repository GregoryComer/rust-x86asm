use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn lfs_1() {
    run_test(&Instruction { mnemonic: Mnemonic::LFS, operand1: Some(Direct(CX)), operand2: Some(Indirect(SI, Some(OperandSize::Far16), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 180, 12], OperandSize::Word)
}

#[test]
fn lfs_2() {
    run_test(&Instruction { mnemonic: Mnemonic::LFS, operand1: Some(Direct(CX)), operand2: Some(IndirectDisplaced(EDX, 311726620, Some(OperandSize::Far16), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 180, 138, 28, 146, 148, 18], OperandSize::Dword)
}

#[test]
fn lfs_3() {
    run_test(&Instruction { mnemonic: Mnemonic::LFS, operand1: Some(Direct(SP)), operand2: Some(Indirect(RBX, Some(OperandSize::Far16), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 180, 35], OperandSize::Qword)
}

#[test]
fn lfs_4() {
    run_test(&Instruction { mnemonic: Mnemonic::LFS, operand1: Some(Direct(ECX)), operand2: Some(IndirectDisplaced(ESI, 1972397956, Some(OperandSize::Far32), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 180, 142, 132, 103, 144, 117], OperandSize::Dword)
}

#[test]
fn lfs_5() {
    run_test(&Instruction { mnemonic: Mnemonic::LFS, operand1: Some(Direct(EBX)), operand2: Some(IndirectDisplaced(RCX, 1179047871, Some(OperandSize::Far32), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 180, 153, 191, 215, 70, 70], OperandSize::Qword)
}

#[test]
fn lfs_6() {
    run_test(&Instruction { mnemonic: Mnemonic::LFS, operand1: Some(Direct(RBP)), operand2: Some(IndirectScaledDisplaced(RBX, Eight, 1577203602, Some(OperandSize::Far64), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 180, 44, 221, 146, 55, 2, 94], OperandSize::Qword)
}

