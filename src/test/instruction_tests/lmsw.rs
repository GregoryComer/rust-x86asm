use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn lmsw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::LMSW, operand1: Some(Direct(DI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 1, 247], OperandSize::Word)
}

fn lmsw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::LMSW, operand1: Some(IndirectDisplaced(BP, 14547, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 1, 182, 211, 56], OperandSize::Word)
}

fn lmsw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::LMSW, operand1: Some(Direct(CX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 1, 241], OperandSize::Dword)
}

fn lmsw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::LMSW, operand1: Some(IndirectScaledDisplaced(EAX, Eight, 406788914, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 1, 52, 197, 50, 27, 63, 24], OperandSize::Dword)
}

fn lmsw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::LMSW, operand1: Some(Direct(SP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 1, 244], OperandSize::Qword)
}

fn lmsw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::LMSW, operand1: Some(Indirect(RAX, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 1, 48], OperandSize::Qword)
}

