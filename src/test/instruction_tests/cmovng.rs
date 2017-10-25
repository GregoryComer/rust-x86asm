use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn cmovng_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNG, operand1: Some(Direct(DX)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 78, 209], OperandSize::Word)
}

fn cmovng_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNG, operand1: Some(Direct(DI)), operand2: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 30507, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 78, 185, 43, 119], OperandSize::Word)
}

fn cmovng_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNG, operand1: Some(Direct(DI)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 78, 255], OperandSize::Dword)
}

fn cmovng_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNG, operand1: Some(Direct(DI)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EBX, Four, 479776930, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 78, 188, 152, 162, 208, 152, 28], OperandSize::Dword)
}

fn cmovng_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNG, operand1: Some(Direct(BP)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 78, 233], OperandSize::Qword)
}

fn cmovng_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNG, operand1: Some(Direct(SP)), operand2: Some(Indirect(RDX, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 78, 34], OperandSize::Qword)
}

fn cmovng_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNG, operand1: Some(Direct(ECX)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 78, 201], OperandSize::Word)
}

fn cmovng_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNG, operand1: Some(Direct(ESP)), operand2: Some(IndirectDisplaced(SI, 31301, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 78, 164, 69, 122], OperandSize::Word)
}

fn cmovng_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNG, operand1: Some(Direct(ESI)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 78, 247], OperandSize::Dword)
}

fn cmovng_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNG, operand1: Some(Direct(EBX)), operand2: Some(Indirect(ECX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 78, 25], OperandSize::Dword)
}

fn cmovng_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNG, operand1: Some(Direct(EBP)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 78, 233], OperandSize::Qword)
}

fn cmovng_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNG, operand1: Some(Direct(ESI)), operand2: Some(IndirectScaledDisplaced(RAX, Two, 1019235791, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 78, 52, 69, 207, 77, 192, 60], OperandSize::Qword)
}

fn cmovng_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNG, operand1: Some(Direct(RBX)), operand2: Some(Direct(RBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 78, 221], OperandSize::Qword)
}

fn cmovng_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNG, operand1: Some(Direct(RDI)), operand2: Some(IndirectDisplaced(RDX, 1373396625, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 78, 186, 145, 94, 220, 81], OperandSize::Qword)
}

