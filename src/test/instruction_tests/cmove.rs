use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn cmove_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVE, operand1: Some(Direct(BX)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 68, 220], OperandSize::Word)
}

fn cmove_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVE, operand1: Some(Direct(BP)), operand2: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 27992, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 68, 169, 88, 109], OperandSize::Word)
}

fn cmove_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVE, operand1: Some(Direct(BX)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 68, 221], OperandSize::Dword)
}

fn cmove_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVE, operand1: Some(Direct(SI)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, ESI, Two, 1685240314, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 68, 180, 115, 250, 185, 114, 100], OperandSize::Dword)
}

fn cmove_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVE, operand1: Some(Direct(CX)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 68, 202], OperandSize::Qword)
}

fn cmove_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVE, operand1: Some(Direct(SP)), operand2: Some(IndirectScaledIndexed(RAX, RBX, Four, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 68, 36, 152], OperandSize::Qword)
}

fn cmove_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVE, operand1: Some(Direct(ECX)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 68, 207], OperandSize::Word)
}

fn cmove_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVE, operand1: Some(Direct(ESI)), operand2: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 17307, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 68, 178, 155, 67], OperandSize::Word)
}

fn cmove_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVE, operand1: Some(Direct(ESP)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 68, 229], OperandSize::Dword)
}

fn cmove_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVE, operand1: Some(Direct(ECX)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EDI, Two, 459127350, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 68, 140, 120, 54, 186, 93, 27], OperandSize::Dword)
}

fn cmove_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVE, operand1: Some(Direct(ESI)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 68, 242], OperandSize::Qword)
}

fn cmove_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVE, operand1: Some(Direct(EDI)), operand2: Some(IndirectDisplaced(RBX, 32822961, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 68, 187, 177, 214, 244, 1], OperandSize::Qword)
}

fn cmove_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVE, operand1: Some(Direct(RCX)), operand2: Some(Direct(RDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 68, 202], OperandSize::Qword)
}

fn cmove_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVE, operand1: Some(Direct(RBX)), operand2: Some(IndirectDisplaced(RAX, 809760301, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 68, 152, 45, 246, 67, 48], OperandSize::Qword)
}

