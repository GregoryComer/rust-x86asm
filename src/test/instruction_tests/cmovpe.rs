use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn cmovpe_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVPE, operand1: Some(Direct(CX)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 74, 201], OperandSize::Word)
}

fn cmovpe_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVPE, operand1: Some(Direct(BX)), operand2: Some(IndirectDisplaced(DI, 4298, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 74, 157, 202, 16], OperandSize::Word)
}

fn cmovpe_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVPE, operand1: Some(Direct(SI)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 74, 241], OperandSize::Dword)
}

fn cmovpe_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVPE, operand1: Some(Direct(SP)), operand2: Some(Indirect(EDI, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 74, 39], OperandSize::Dword)
}

fn cmovpe_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVPE, operand1: Some(Direct(BX)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 74, 219], OperandSize::Qword)
}

fn cmovpe_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVPE, operand1: Some(Direct(BX)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RCX, Four, 2046333172, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 74, 156, 136, 244, 144, 248, 121], OperandSize::Qword)
}

fn cmovpe_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVPE, operand1: Some(Direct(EBX)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 74, 223], OperandSize::Word)
}

fn cmovpe_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVPE, operand1: Some(Direct(EBX)), operand2: Some(IndirectScaledIndexed(BX, DI, One, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 74, 25], OperandSize::Word)
}

fn cmovpe_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVPE, operand1: Some(Direct(EDI)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 74, 253], OperandSize::Dword)
}

fn cmovpe_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVPE, operand1: Some(Direct(ESP)), operand2: Some(IndirectDisplaced(EAX, 133358418, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 74, 160, 82, 227, 242, 7], OperandSize::Dword)
}

fn cmovpe_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVPE, operand1: Some(Direct(ESP)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 74, 227], OperandSize::Qword)
}

fn cmovpe_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVPE, operand1: Some(Direct(ESI)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RDX, Eight, 648913050, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 74, 180, 209, 154, 160, 173, 38], OperandSize::Qword)
}

fn cmovpe_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVPE, operand1: Some(Direct(RCX)), operand2: Some(Direct(RBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 74, 203], OperandSize::Qword)
}

fn cmovpe_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVPE, operand1: Some(Direct(RBX)), operand2: Some(IndirectDisplaced(RDI, 1597612610, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 74, 159, 66, 162, 57, 95], OperandSize::Qword)
}

