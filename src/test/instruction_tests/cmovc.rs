use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn cmovc_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVC, operand1: Some(Direct(CX)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 66, 204], OperandSize::Word)
}

fn cmovc_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVC, operand1: Some(Direct(DI)), operand2: Some(Indirect(DI, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 66, 61], OperandSize::Word)
}

fn cmovc_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVC, operand1: Some(Direct(SI)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 66, 245], OperandSize::Dword)
}

fn cmovc_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVC, operand1: Some(Direct(CX)), operand2: Some(IndirectScaledIndexed(EDI, EBX, Eight, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 66, 12, 223], OperandSize::Dword)
}

fn cmovc_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVC, operand1: Some(Direct(DI)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 66, 252], OperandSize::Qword)
}

fn cmovc_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVC, operand1: Some(Direct(DI)), operand2: Some(IndirectScaledIndexed(RDI, RCX, Four, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 66, 60, 143], OperandSize::Qword)
}

fn cmovc_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVC, operand1: Some(Direct(EBX)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 66, 219], OperandSize::Word)
}

fn cmovc_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVC, operand1: Some(Direct(EBX)), operand2: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 566, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 66, 152, 54, 2], OperandSize::Word)
}

fn cmovc_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVC, operand1: Some(Direct(ESP)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 66, 231], OperandSize::Dword)
}

fn cmovc_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVC, operand1: Some(Direct(EDX)), operand2: Some(Indirect(EAX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 66, 16], OperandSize::Dword)
}

fn cmovc_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVC, operand1: Some(Direct(ESP)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 66, 226], OperandSize::Qword)
}

fn cmovc_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVC, operand1: Some(Direct(EDX)), operand2: Some(IndirectDisplaced(RCX, 1275361678, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 66, 145, 142, 121, 4, 76], OperandSize::Qword)
}

fn cmovc_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVC, operand1: Some(Direct(RBX)), operand2: Some(Direct(RSP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 66, 220], OperandSize::Qword)
}

fn cmovc_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVC, operand1: Some(Direct(RDI)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RSI, Two, 2049716729, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 66, 188, 113, 249, 49, 44, 122], OperandSize::Qword)
}

