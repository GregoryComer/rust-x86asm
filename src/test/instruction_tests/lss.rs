use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn lss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::LSS, operand1: Some(Direct(SI)), operand2: Some(IndirectDisplaced(DI, 4198, Some(OperandSize::Far16), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 178, 181, 102, 16], OperandSize::Word)
}

fn lss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::LSS, operand1: Some(Direct(DX)), operand2: Some(IndirectDisplaced(EBX, 2051331775, Some(OperandSize::Far16), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 178, 147, 191, 214, 68, 122], OperandSize::Dword)
}

fn lss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::LSS, operand1: Some(Direct(CX)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RCX, Four, 74583810, Some(OperandSize::Far16), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 178, 140, 143, 2, 15, 114, 4], OperandSize::Qword)
}

fn lss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::LSS, operand1: Some(Direct(EDX)), operand2: Some(IndirectScaledDisplaced(EDI, Two, 564960042, Some(OperandSize::Far32), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 178, 20, 125, 42, 155, 172, 33], OperandSize::Dword)
}

fn lss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::LSS, operand1: Some(Direct(EDX)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RBX, Four, 1908731456, Some(OperandSize::Far32), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 178, 148, 153, 64, 238, 196, 113], OperandSize::Qword)
}

fn lss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::LSS, operand1: Some(Direct(RDI)), operand2: Some(IndirectScaledIndexed(RBX, RDI, Two, Some(OperandSize::Far64), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 178, 60, 123], OperandSize::Qword)
}

