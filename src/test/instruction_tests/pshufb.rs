use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn pshufb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFB, operand1: Some(Direct(MM3)), operand2: Some(Direct(MM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 0, 219], OperandSize::Dword)
}

fn pshufb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFB, operand1: Some(Direct(MM3)), operand2: Some(IndirectScaledIndexed(EDX, ECX, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 0, 28, 138], OperandSize::Dword)
}

fn pshufb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFB, operand1: Some(Direct(MM1)), operand2: Some(Direct(MM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 0, 200], OperandSize::Qword)
}

fn pshufb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFB, operand1: Some(Direct(MM5)), operand2: Some(IndirectDisplaced(RDI, 545769040, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 0, 175, 80, 198, 135, 32], OperandSize::Qword)
}

fn pshufb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 0, 202], OperandSize::Dword)
}

fn pshufb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFB, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledDisplaced(EAX, Two, 1542854032, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 0, 60, 69, 144, 21, 246, 91], OperandSize::Dword)
}

fn pshufb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 0, 241], OperandSize::Qword)
}

fn pshufb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFB, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledDisplaced(RDI, Eight, 579917817, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 0, 12, 253, 249, 215, 144, 34], OperandSize::Qword)
}

