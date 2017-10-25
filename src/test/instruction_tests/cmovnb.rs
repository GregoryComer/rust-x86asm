use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn cmovnb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNB, operand1: Some(Direct(BP)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 67, 236], OperandSize::Word)
}

fn cmovnb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNB, operand1: Some(Direct(CX)), operand2: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 152, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 67, 139, 152, 0], OperandSize::Word)
}

fn cmovnb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNB, operand1: Some(Direct(SI)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 67, 245], OperandSize::Dword)
}

fn cmovnb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNB, operand1: Some(Direct(BX)), operand2: Some(IndirectScaledDisplaced(EAX, Four, 1509426750, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 67, 28, 133, 62, 6, 248, 89], OperandSize::Dword)
}

fn cmovnb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNB, operand1: Some(Direct(BX)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 67, 220], OperandSize::Qword)
}

fn cmovnb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNB, operand1: Some(Direct(CX)), operand2: Some(IndirectScaledIndexed(RDX, RCX, Four, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 67, 12, 138], OperandSize::Qword)
}

fn cmovnb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNB, operand1: Some(Direct(ECX)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 67, 202], OperandSize::Word)
}

fn cmovnb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNB, operand1: Some(Direct(ECX)), operand2: Some(IndirectDisplaced(BP, 130, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 67, 142, 130, 0], OperandSize::Word)
}

fn cmovnb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNB, operand1: Some(Direct(EBP)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 67, 237], OperandSize::Dword)
}

fn cmovnb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNB, operand1: Some(Direct(ESP)), operand2: Some(IndirectScaledIndexed(EDI, EAX, Four, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 67, 36, 135], OperandSize::Dword)
}

fn cmovnb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNB, operand1: Some(Direct(EBX)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 67, 220], OperandSize::Qword)
}

fn cmovnb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNB, operand1: Some(Direct(EDI)), operand2: Some(Indirect(RDX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 67, 58], OperandSize::Qword)
}

fn cmovnb_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNB, operand1: Some(Direct(RCX)), operand2: Some(Direct(RSI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 67, 206], OperandSize::Qword)
}

fn cmovnb_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNB, operand1: Some(Direct(RBP)), operand2: Some(IndirectDisplaced(RSI, 1398017651, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 67, 174, 115, 14, 84, 83], OperandSize::Qword)
}

