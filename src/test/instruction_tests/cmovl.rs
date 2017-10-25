use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn cmovl_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVL, operand1: Some(Direct(SP)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 76, 229], OperandSize::Word)
}

fn cmovl_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVL, operand1: Some(Direct(DI)), operand2: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 10704, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 76, 184, 208, 41], OperandSize::Word)
}

fn cmovl_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVL, operand1: Some(Direct(DX)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 76, 215], OperandSize::Dword)
}

fn cmovl_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVL, operand1: Some(Direct(DX)), operand2: Some(Indirect(EDI, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 76, 23], OperandSize::Dword)
}

fn cmovl_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVL, operand1: Some(Direct(SP)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 76, 227], OperandSize::Qword)
}

fn cmovl_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVL, operand1: Some(Direct(SI)), operand2: Some(IndirectScaledIndexed(RAX, RSI, Four, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 76, 52, 176], OperandSize::Qword)
}

fn cmovl_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVL, operand1: Some(Direct(EBP)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 76, 236], OperandSize::Word)
}

fn cmovl_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVL, operand1: Some(Direct(EBP)), operand2: Some(Indirect(BX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 76, 47], OperandSize::Word)
}

fn cmovl_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVL, operand1: Some(Direct(ECX)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 76, 202], OperandSize::Dword)
}

fn cmovl_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVL, operand1: Some(Direct(EDX)), operand2: Some(Indirect(EDI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 76, 23], OperandSize::Dword)
}

fn cmovl_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVL, operand1: Some(Direct(EBP)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 76, 237], OperandSize::Qword)
}

fn cmovl_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVL, operand1: Some(Direct(EBX)), operand2: Some(IndirectDisplaced(RSI, 742652997, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 76, 158, 69, 252, 67, 44], OperandSize::Qword)
}

fn cmovl_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVL, operand1: Some(Direct(RDI)), operand2: Some(Direct(RBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 76, 253], OperandSize::Qword)
}

fn cmovl_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVL, operand1: Some(Direct(RSI)), operand2: Some(IndirectDisplaced(RDI, 1044138253, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 76, 183, 13, 73, 60, 62], OperandSize::Qword)
}

