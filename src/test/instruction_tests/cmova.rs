use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn cmova_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVA, operand1: Some(Direct(BX)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 71, 223], OperandSize::Word)
}

fn cmova_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVA, operand1: Some(Direct(DX)), operand2: Some(IndirectScaledIndexed(BX, SI, One, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 71, 16], OperandSize::Word)
}

fn cmova_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVA, operand1: Some(Direct(BX)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 71, 222], OperandSize::Dword)
}

fn cmova_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVA, operand1: Some(Direct(SP)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, ECX, Eight, 1531284144, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 71, 164, 200, 176, 138, 69, 91], OperandSize::Dword)
}

fn cmova_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVA, operand1: Some(Direct(CX)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 71, 205], OperandSize::Qword)
}

fn cmova_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVA, operand1: Some(Direct(DI)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RBX, Two, 435374238, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 71, 188, 95, 158, 72, 243, 25], OperandSize::Qword)
}

fn cmova_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVA, operand1: Some(Direct(ECX)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 71, 204], OperandSize::Word)
}

fn cmova_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVA, operand1: Some(Direct(EBX)), operand2: Some(IndirectDisplaced(BP, 20732, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 71, 158, 252, 80], OperandSize::Word)
}

fn cmova_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVA, operand1: Some(Direct(ESI)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 71, 245], OperandSize::Dword)
}

fn cmova_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVA, operand1: Some(Direct(ESI)), operand2: Some(IndirectScaledIndexed(ECX, EBX, Two, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 71, 52, 89], OperandSize::Dword)
}

fn cmova_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVA, operand1: Some(Direct(EDI)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 71, 251], OperandSize::Qword)
}

fn cmova_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVA, operand1: Some(Direct(EBX)), operand2: Some(IndirectScaledDisplaced(RDX, Eight, 1607259067, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 71, 28, 213, 187, 211, 204, 95], OperandSize::Qword)
}

fn cmova_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVA, operand1: Some(Direct(RDX)), operand2: Some(Direct(RSP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 71, 212], OperandSize::Qword)
}

fn cmova_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVA, operand1: Some(Direct(RCX)), operand2: Some(Indirect(RDX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 71, 10], OperandSize::Qword)
}

