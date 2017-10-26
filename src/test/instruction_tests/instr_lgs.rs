use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn lgs_1() {
    run_test(&Instruction { mnemonic: Mnemonic::LGS, operand1: Some(Direct(SI)), operand2: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 97, Some(OperandSize::Far16), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 181, 114, 97], OperandSize::Word)
}

#[test]
fn lgs_2() {
    run_test(&Instruction { mnemonic: Mnemonic::LGS, operand1: Some(Direct(BX)), operand2: Some(IndirectDisplaced(EAX, 268452467, Some(OperandSize::Far16), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 181, 152, 115, 66, 0, 16], OperandSize::Dword)
}

#[test]
fn lgs_3() {
    run_test(&Instruction { mnemonic: Mnemonic::LGS, operand1: Some(Direct(BP)), operand2: Some(Indirect(RDX, Some(OperandSize::Far16), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 181, 42], OperandSize::Qword)
}

#[test]
fn lgs_4() {
    run_test(&Instruction { mnemonic: Mnemonic::LGS, operand1: Some(Direct(EBX)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EBX, Two, 560665316, Some(OperandSize::Far32), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 181, 156, 95, 228, 18, 107, 33], OperandSize::Dword)
}

#[test]
fn lgs_5() {
    run_test(&Instruction { mnemonic: Mnemonic::LGS, operand1: Some(Direct(EBP)), operand2: Some(IndirectScaledIndexed(RDX, RSI, Eight, Some(OperandSize::Far32), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 181, 44, 242], OperandSize::Qword)
}

#[test]
fn lgs_6() {
    run_test(&Instruction { mnemonic: Mnemonic::LGS, operand1: Some(Direct(RBX)), operand2: Some(IndirectDisplaced(RDX, 672327322, Some(OperandSize::Far64), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 181, 154, 154, 230, 18, 40], OperandSize::Qword)
}

