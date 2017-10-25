use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn cmovnge_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNGE, operand1: Some(Direct(SP)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 76, 228], OperandSize::Word)
}

fn cmovnge_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNGE, operand1: Some(Direct(BP)), operand2: Some(Indirect(BX, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 76, 47], OperandSize::Word)
}

fn cmovnge_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNGE, operand1: Some(Direct(DX)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 76, 214], OperandSize::Dword)
}

fn cmovnge_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNGE, operand1: Some(Direct(SI)), operand2: Some(Indirect(EDX, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 76, 50], OperandSize::Dword)
}

fn cmovnge_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNGE, operand1: Some(Direct(BX)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 76, 221], OperandSize::Qword)
}

fn cmovnge_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNGE, operand1: Some(Direct(DX)), operand2: Some(IndirectDisplaced(RSI, 307745240, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 76, 150, 216, 209, 87, 18], OperandSize::Qword)
}

fn cmovnge_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNGE, operand1: Some(Direct(ESI)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 76, 246], OperandSize::Word)
}

fn cmovnge_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNGE, operand1: Some(Direct(ECX)), operand2: Some(IndirectDisplaced(BX, 8914, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 76, 143, 210, 34], OperandSize::Word)
}

fn cmovnge_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNGE, operand1: Some(Direct(ESP)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 76, 231], OperandSize::Dword)
}

fn cmovnge_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNGE, operand1: Some(Direct(EBX)), operand2: Some(IndirectDisplaced(ECX, 2099078369, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 76, 153, 225, 100, 29, 125], OperandSize::Dword)
}

fn cmovnge_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNGE, operand1: Some(Direct(ESP)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 76, 226], OperandSize::Qword)
}

fn cmovnge_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNGE, operand1: Some(Direct(EDX)), operand2: Some(IndirectScaledIndexed(RAX, RBX, Eight, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 76, 20, 216], OperandSize::Qword)
}

fn cmovnge_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNGE, operand1: Some(Direct(RBP)), operand2: Some(Direct(RSP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 76, 236], OperandSize::Qword)
}

fn cmovnge_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNGE, operand1: Some(Direct(RDI)), operand2: Some(Indirect(RSI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 76, 62], OperandSize::Qword)
}

