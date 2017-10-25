use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn cmovb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVB, operand1: Some(Direct(SI)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 66, 241], OperandSize::Word)
}

fn cmovb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVB, operand1: Some(Direct(BX)), operand2: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 68, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 66, 90, 68], OperandSize::Word)
}

fn cmovb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVB, operand1: Some(Direct(BP)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 66, 236], OperandSize::Dword)
}

fn cmovb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVB, operand1: Some(Direct(DI)), operand2: Some(IndirectDisplaced(EAX, 1240678133, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 66, 184, 245, 62, 243, 73], OperandSize::Dword)
}

fn cmovb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVB, operand1: Some(Direct(BP)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 66, 239], OperandSize::Qword)
}

fn cmovb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVB, operand1: Some(Direct(SP)), operand2: Some(Indirect(RDX, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 66, 34], OperandSize::Qword)
}

fn cmovb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVB, operand1: Some(Direct(EDX)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 66, 212], OperandSize::Word)
}

fn cmovb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVB, operand1: Some(Direct(ECX)), operand2: Some(IndirectScaledIndexed(BX, DI, One, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 66, 9], OperandSize::Word)
}

fn cmovb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVB, operand1: Some(Direct(EDI)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 66, 254], OperandSize::Dword)
}

fn cmovb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVB, operand1: Some(Direct(ESP)), operand2: Some(IndirectScaledDisplaced(EAX, Four, 168887598, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 66, 36, 133, 46, 5, 17, 10], OperandSize::Dword)
}

fn cmovb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVB, operand1: Some(Direct(ESI)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 66, 245], OperandSize::Qword)
}

fn cmovb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVB, operand1: Some(Direct(EDI)), operand2: Some(IndirectScaledIndexed(RCX, RDX, Two, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 66, 60, 81], OperandSize::Qword)
}

fn cmovb_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVB, operand1: Some(Direct(RDI)), operand2: Some(Direct(RCX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 66, 249], OperandSize::Qword)
}

fn cmovb_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVB, operand1: Some(Direct(RBP)), operand2: Some(Indirect(RDX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 66, 42], OperandSize::Qword)
}

