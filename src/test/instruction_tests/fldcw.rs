use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn fldcw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FLDCW, operand1: Some(Memory(3030, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 46, 214, 11], OperandSize::Word)
}

fn fldcw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FLDCW, operand1: Some(IndirectScaledIndexed(EBX, ECX, Eight, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 44, 203], OperandSize::Dword)
}

fn fldcw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FLDCW, operand1: Some(Indirect(RDI, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 47], OperandSize::Qword)
}

