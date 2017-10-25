use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmpxchg_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG, operand1: Some(Direct(CL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 176, 201], OperandSize::Word)
}

#[test]
fn cmpxchg_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG, operand1: Some(Memory(22145, Some(OperandSize::Byte), None)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 176, 22, 129, 86], OperandSize::Word)
}

#[test]
fn cmpxchg_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG, operand1: Some(Direct(BL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 176, 219], OperandSize::Dword)
}

#[test]
fn cmpxchg_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG, operand1: Some(IndirectDisplaced(ECX, 1014484780, Some(OperandSize::Byte), None)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 176, 145, 44, 207, 119, 60], OperandSize::Dword)
}

#[test]
fn cmpxchg_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG, operand1: Some(Direct(BL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 176, 203], OperandSize::Qword)
}

#[test]
fn cmpxchg_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG, operand1: Some(IndirectScaledIndexed(RAX, RDI, Four, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 176, 12, 184], OperandSize::Qword)
}

#[test]
fn cmpxchg_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG, operand1: Some(Direct(DL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 176, 202], OperandSize::Qword)
}

#[test]
fn cmpxchg_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG, operand1: Some(IndirectScaledIndexed(RDX, RCX, Two, Some(OperandSize::Byte), None)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 176, 28, 74], OperandSize::Qword)
}

#[test]
fn cmpxchg_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG, operand1: Some(Direct(DX)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 177, 242], OperandSize::Word)
}

#[test]
fn cmpxchg_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 32248, Some(OperandSize::Word), None)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 177, 152, 248, 125], OperandSize::Word)
}

#[test]
fn cmpxchg_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG, operand1: Some(Direct(SP)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 177, 212], OperandSize::Dword)
}

#[test]
fn cmpxchg_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG, operand1: Some(Indirect(ESI, Some(OperandSize::Word), None)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 177, 62], OperandSize::Dword)
}

#[test]
fn cmpxchg_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG, operand1: Some(Direct(DX)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 177, 202], OperandSize::Qword)
}

#[test]
fn cmpxchg_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG, operand1: Some(IndirectDisplaced(RDI, 658411151, Some(OperandSize::Word), None)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 177, 167, 143, 142, 62, 39], OperandSize::Qword)
}

#[test]
fn cmpxchg_15() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG, operand1: Some(Direct(EBX)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 177, 251], OperandSize::Word)
}

#[test]
fn cmpxchg_16() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG, operand1: Some(Indirect(SI, Some(OperandSize::Dword), None)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 177, 44], OperandSize::Word)
}

#[test]
fn cmpxchg_17() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG, operand1: Some(Direct(ESI)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 177, 222], OperandSize::Dword)
}

#[test]
fn cmpxchg_18() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG, operand1: Some(IndirectScaledDisplaced(EBX, Eight, 2121177071, Some(OperandSize::Dword), None)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 177, 20, 221, 239, 151, 110, 126], OperandSize::Dword)
}

#[test]
fn cmpxchg_19() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG, operand1: Some(Direct(EDI)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 177, 215], OperandSize::Qword)
}

#[test]
fn cmpxchg_20() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG, operand1: Some(IndirectScaledIndexed(RCX, RDI, Eight, Some(OperandSize::Dword), None)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 177, 36, 249], OperandSize::Qword)
}

#[test]
fn cmpxchg_21() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG, operand1: Some(Direct(RDX)), operand2: Some(Direct(RBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 177, 218], OperandSize::Qword)
}

#[test]
fn cmpxchg_22() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG, operand1: Some(IndirectScaledIndexed(RDX, RBX, Eight, Some(OperandSize::Qword), None)), operand2: Some(Direct(RCX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 177, 12, 218], OperandSize::Qword)
}

