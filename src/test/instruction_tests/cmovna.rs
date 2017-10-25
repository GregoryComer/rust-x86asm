use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn cmovna_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNA, operand1: Some(Direct(CX)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 70, 203], OperandSize::Word)
}

fn cmovna_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNA, operand1: Some(Direct(DI)), operand2: Some(IndirectDisplaced(DI, 26896, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 70, 189, 16, 105], OperandSize::Word)
}

fn cmovna_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNA, operand1: Some(Direct(SI)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 70, 245], OperandSize::Dword)
}

fn cmovna_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNA, operand1: Some(Direct(SP)), operand2: Some(IndirectScaledDisplaced(EDI, Eight, 1490308207, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 70, 36, 253, 111, 76, 212, 88], OperandSize::Dword)
}

fn cmovna_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNA, operand1: Some(Direct(DX)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 70, 211], OperandSize::Qword)
}

fn cmovna_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNA, operand1: Some(Direct(SP)), operand2: Some(IndirectScaledDisplaced(RSI, Four, 2007046754, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 70, 36, 181, 98, 26, 161, 119], OperandSize::Qword)
}

fn cmovna_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNA, operand1: Some(Direct(EDI)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 70, 251], OperandSize::Word)
}

fn cmovna_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNA, operand1: Some(Direct(ECX)), operand2: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 12840, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 70, 139, 40, 50], OperandSize::Word)
}

fn cmovna_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNA, operand1: Some(Direct(EBP)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 70, 239], OperandSize::Dword)
}

fn cmovna_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNA, operand1: Some(Direct(ESI)), operand2: Some(IndirectScaledDisplaced(ESI, Four, 728518023, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 70, 52, 181, 135, 77, 108, 43], OperandSize::Dword)
}

fn cmovna_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNA, operand1: Some(Direct(ESP)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 70, 227], OperandSize::Qword)
}

fn cmovna_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNA, operand1: Some(Direct(EDI)), operand2: Some(IndirectScaledIndexed(RDX, RCX, Eight, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 70, 60, 202], OperandSize::Qword)
}

fn cmovna_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNA, operand1: Some(Direct(RCX)), operand2: Some(Direct(RDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 70, 202], OperandSize::Qword)
}

fn cmovna_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNA, operand1: Some(Direct(RDX)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RBX, Four, 345285568, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 70, 148, 155, 192, 163, 148, 20], OperandSize::Qword)
}

