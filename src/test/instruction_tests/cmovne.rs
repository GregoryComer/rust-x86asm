use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn cmovne_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNE, operand1: Some(Direct(SP)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 69, 229], OperandSize::Word)
}

fn cmovne_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNE, operand1: Some(Direct(DX)), operand2: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 203, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 69, 147, 203, 0], OperandSize::Word)
}

fn cmovne_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNE, operand1: Some(Direct(DX)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 69, 214], OperandSize::Dword)
}

fn cmovne_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNE, operand1: Some(Direct(BP)), operand2: Some(IndirectScaledIndexed(EBX, EAX, Four, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 69, 44, 131], OperandSize::Dword)
}

fn cmovne_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNE, operand1: Some(Direct(BX)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 69, 219], OperandSize::Qword)
}

fn cmovne_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNE, operand1: Some(Direct(DI)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RCX, Four, 1541080292, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 69, 188, 143, 228, 4, 219, 91], OperandSize::Qword)
}

fn cmovne_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNE, operand1: Some(Direct(ECX)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 69, 202], OperandSize::Word)
}

fn cmovne_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNE, operand1: Some(Direct(ESI)), operand2: Some(IndirectDisplaced(DI, 26158, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 69, 181, 46, 102], OperandSize::Word)
}

fn cmovne_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNE, operand1: Some(Direct(EBP)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 69, 233], OperandSize::Dword)
}

fn cmovne_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNE, operand1: Some(Direct(EBP)), operand2: Some(IndirectDisplaced(ESI, 2042751067, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 69, 174, 91, 232, 193, 121], OperandSize::Dword)
}

fn cmovne_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNE, operand1: Some(Direct(ESP)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 69, 229], OperandSize::Qword)
}

fn cmovne_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNE, operand1: Some(Direct(ECX)), operand2: Some(Indirect(RDX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 69, 10], OperandSize::Qword)
}

fn cmovne_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNE, operand1: Some(Direct(RCX)), operand2: Some(Direct(RDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 69, 207], OperandSize::Qword)
}

fn cmovne_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNE, operand1: Some(Direct(RBP)), operand2: Some(IndirectDisplaced(RAX, 477254473, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 69, 168, 73, 83, 114, 28], OperandSize::Qword)
}

