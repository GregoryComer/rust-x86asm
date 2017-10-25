use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn bsr_1() {
    run_test(&Instruction { mnemonic: Mnemonic::BSR, operand1: Some(Direct(DI)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 189, 255], OperandSize::Word)
}

fn bsr_2() {
    run_test(&Instruction { mnemonic: Mnemonic::BSR, operand1: Some(Direct(SP)), operand2: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 17427, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 189, 161, 19, 68], OperandSize::Word)
}

fn bsr_3() {
    run_test(&Instruction { mnemonic: Mnemonic::BSR, operand1: Some(Direct(SP)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 189, 228], OperandSize::Dword)
}

fn bsr_4() {
    run_test(&Instruction { mnemonic: Mnemonic::BSR, operand1: Some(Direct(BX)), operand2: Some(Indirect(ESI, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 189, 30], OperandSize::Dword)
}

fn bsr_5() {
    run_test(&Instruction { mnemonic: Mnemonic::BSR, operand1: Some(Direct(DI)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 189, 254], OperandSize::Qword)
}

fn bsr_6() {
    run_test(&Instruction { mnemonic: Mnemonic::BSR, operand1: Some(Direct(BX)), operand2: Some(IndirectScaledDisplaced(RAX, Eight, 250154482, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 189, 28, 197, 242, 13, 233, 14], OperandSize::Qword)
}

fn bsr_7() {
    run_test(&Instruction { mnemonic: Mnemonic::BSR, operand1: Some(Direct(EDX)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 189, 209], OperandSize::Word)
}

fn bsr_8() {
    run_test(&Instruction { mnemonic: Mnemonic::BSR, operand1: Some(Direct(ECX)), operand2: Some(IndirectDisplaced(DI, 2, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 189, 77, 2], OperandSize::Word)
}

fn bsr_9() {
    run_test(&Instruction { mnemonic: Mnemonic::BSR, operand1: Some(Direct(EBX)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 189, 217], OperandSize::Dword)
}

fn bsr_10() {
    run_test(&Instruction { mnemonic: Mnemonic::BSR, operand1: Some(Direct(ESP)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EAX, Two, 1513633415, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 189, 164, 64, 135, 54, 56, 90], OperandSize::Dword)
}

fn bsr_11() {
    run_test(&Instruction { mnemonic: Mnemonic::BSR, operand1: Some(Direct(EDI)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 189, 252], OperandSize::Qword)
}

fn bsr_12() {
    run_test(&Instruction { mnemonic: Mnemonic::BSR, operand1: Some(Direct(EBP)), operand2: Some(IndirectDisplaced(RCX, 731775083, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 189, 169, 107, 0, 158, 43], OperandSize::Qword)
}

fn bsr_13() {
    run_test(&Instruction { mnemonic: Mnemonic::BSR, operand1: Some(Direct(RSP)), operand2: Some(Direct(RSP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 189, 228], OperandSize::Qword)
}

fn bsr_14() {
    run_test(&Instruction { mnemonic: Mnemonic::BSR, operand1: Some(Direct(RDI)), operand2: Some(IndirectDisplaced(RAX, 603179374, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 189, 184, 110, 201, 243, 35], OperandSize::Qword)
}

