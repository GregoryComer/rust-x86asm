use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn lgs_1() {
    run_test(&Instruction { mnemonic: Mnemonic::LGS, operand1: Some(Direct(BX)), operand2: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 32317, Some(OperandSize::Far16), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 181, 154, 61, 126], OperandSize::Word)
}

#[test]
fn lgs_2() {
    run_test(&Instruction { mnemonic: Mnemonic::LGS, operand1: Some(Direct(DX)), operand2: Some(IndirectDisplaced(EBX, 71239161, Some(OperandSize::Far16), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 181, 147, 249, 5, 63, 4], OperandSize::Dword)
}

#[test]
fn lgs_3() {
    run_test(&Instruction { mnemonic: Mnemonic::LGS, operand1: Some(Direct(BP)), operand2: Some(Indirect(RAX, Some(OperandSize::Far16), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 181, 40], OperandSize::Qword)
}

#[test]
fn lgs_4() {
    run_test(&Instruction { mnemonic: Mnemonic::LGS, operand1: Some(Direct(EBP)), operand2: Some(IndirectScaledDisplaced(ECX, Two, 251650835, Some(OperandSize::Far32), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 181, 44, 77, 19, 227, 255, 14], OperandSize::Dword)
}

#[test]
fn lgs_5() {
    run_test(&Instruction { mnemonic: Mnemonic::LGS, operand1: Some(Direct(EDX)), operand2: Some(IndirectScaledDisplaced(RBX, Eight, 573075216, Some(OperandSize::Far32), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 181, 20, 221, 16, 111, 40, 34], OperandSize::Qword)
}

#[test]
fn lgs_6() {
    run_test(&Instruction { mnemonic: Mnemonic::LGS, operand1: Some(Direct(RCX)), operand2: Some(IndirectScaledDisplaced(RDX, Four, 1390988172, Some(OperandSize::Far64), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 181, 12, 149, 140, 203, 232, 82], OperandSize::Qword)
}

