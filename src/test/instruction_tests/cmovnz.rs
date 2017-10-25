use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn cmovnz_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNZ, operand1: Some(Direct(SI)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 69, 241], OperandSize::Word)
}

fn cmovnz_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNZ, operand1: Some(Direct(DI)), operand2: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 28055, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 69, 184, 151, 109], OperandSize::Word)
}

fn cmovnz_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNZ, operand1: Some(Direct(SI)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 69, 244], OperandSize::Dword)
}

fn cmovnz_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNZ, operand1: Some(Direct(SP)), operand2: Some(IndirectDisplaced(EAX, 1851938555, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 69, 160, 251, 86, 98, 110], OperandSize::Dword)
}

fn cmovnz_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNZ, operand1: Some(Direct(SP)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 69, 227], OperandSize::Qword)
}

fn cmovnz_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNZ, operand1: Some(Direct(SI)), operand2: Some(IndirectScaledDisplaced(RAX, Two, 101969343, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 69, 52, 69, 191, 237, 19, 6], OperandSize::Qword)
}

fn cmovnz_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNZ, operand1: Some(Direct(ESI)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 69, 242], OperandSize::Word)
}

fn cmovnz_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNZ, operand1: Some(Direct(ESP)), operand2: Some(IndirectScaledIndexed(BX, DI, One, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 69, 33], OperandSize::Word)
}

fn cmovnz_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNZ, operand1: Some(Direct(EDI)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 69, 251], OperandSize::Dword)
}

fn cmovnz_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNZ, operand1: Some(Direct(ESP)), operand2: Some(Indirect(EDX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 69, 34], OperandSize::Dword)
}

fn cmovnz_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNZ, operand1: Some(Direct(ESI)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 69, 242], OperandSize::Qword)
}

fn cmovnz_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNZ, operand1: Some(Direct(ESI)), operand2: Some(IndirectScaledIndexed(RSI, RDI, Two, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 69, 52, 126], OperandSize::Qword)
}

fn cmovnz_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNZ, operand1: Some(Direct(RDX)), operand2: Some(Direct(RDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 69, 215], OperandSize::Qword)
}

fn cmovnz_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNZ, operand1: Some(Direct(RDX)), operand2: Some(IndirectScaledIndexed(RDI, RDX, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 69, 20, 151], OperandSize::Qword)
}

