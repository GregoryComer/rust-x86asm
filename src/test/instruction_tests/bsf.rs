use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn bsf_1() {
    run_test(&Instruction { mnemonic: Mnemonic::BSF, operand1: Some(Direct(SP)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 188, 226], OperandSize::Word)
}

fn bsf_2() {
    run_test(&Instruction { mnemonic: Mnemonic::BSF, operand1: Some(Direct(CX)), operand2: Some(Indirect(DI, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 188, 13], OperandSize::Word)
}

fn bsf_3() {
    run_test(&Instruction { mnemonic: Mnemonic::BSF, operand1: Some(Direct(CX)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 188, 206], OperandSize::Dword)
}

fn bsf_4() {
    run_test(&Instruction { mnemonic: Mnemonic::BSF, operand1: Some(Direct(BP)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, ESI, Eight, 1725373957, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 188, 172, 240, 5, 30, 215, 102], OperandSize::Dword)
}

fn bsf_5() {
    run_test(&Instruction { mnemonic: Mnemonic::BSF, operand1: Some(Direct(SI)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 188, 242], OperandSize::Qword)
}

fn bsf_6() {
    run_test(&Instruction { mnemonic: Mnemonic::BSF, operand1: Some(Direct(CX)), operand2: Some(Indirect(RDX, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 188, 10], OperandSize::Qword)
}

fn bsf_7() {
    run_test(&Instruction { mnemonic: Mnemonic::BSF, operand1: Some(Direct(ESI)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 188, 247], OperandSize::Word)
}

fn bsf_8() {
    run_test(&Instruction { mnemonic: Mnemonic::BSF, operand1: Some(Direct(ESI)), operand2: Some(IndirectDisplaced(BX, 3005, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 188, 183, 189, 11], OperandSize::Word)
}

fn bsf_9() {
    run_test(&Instruction { mnemonic: Mnemonic::BSF, operand1: Some(Direct(EBX)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 188, 221], OperandSize::Dword)
}

fn bsf_10() {
    run_test(&Instruction { mnemonic: Mnemonic::BSF, operand1: Some(Direct(ESI)), operand2: Some(IndirectScaledIndexed(ECX, EDI, Eight, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 188, 52, 249], OperandSize::Dword)
}

fn bsf_11() {
    run_test(&Instruction { mnemonic: Mnemonic::BSF, operand1: Some(Direct(EBP)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 188, 235], OperandSize::Qword)
}

fn bsf_12() {
    run_test(&Instruction { mnemonic: Mnemonic::BSF, operand1: Some(Direct(ECX)), operand2: Some(Indirect(RBX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 188, 11], OperandSize::Qword)
}

fn bsf_13() {
    run_test(&Instruction { mnemonic: Mnemonic::BSF, operand1: Some(Direct(RCX)), operand2: Some(Direct(RCX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 188, 201], OperandSize::Qword)
}

fn bsf_14() {
    run_test(&Instruction { mnemonic: Mnemonic::BSF, operand1: Some(Direct(RBX)), operand2: Some(IndirectScaledDisplaced(RSI, Four, 701097843, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 188, 28, 181, 115, 231, 201, 41], OperandSize::Qword)
}

