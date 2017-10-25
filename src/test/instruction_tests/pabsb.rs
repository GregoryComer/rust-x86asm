use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn pabsb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSB, operand1: Some(Direct(MM0)), operand2: Some(Direct(MM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 28, 193], OperandSize::Dword)
}

fn pabsb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSB, operand1: Some(Direct(MM1)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, ESI, Two, 894900722, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 28, 140, 112, 242, 25, 87, 53], OperandSize::Dword)
}

fn pabsb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSB, operand1: Some(Direct(MM1)), operand2: Some(Direct(MM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 28, 206], OperandSize::Qword)
}

fn pabsb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSB, operand1: Some(Direct(MM2)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RBX, Four, 145429941, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 28, 148, 154, 181, 21, 171, 8], OperandSize::Qword)
}

fn pabsb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSB, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 28, 211], OperandSize::Dword)
}

fn pabsb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSB, operand1: Some(Direct(XMM4)), operand2: Some(IndirectDisplaced(EDI, 1792153389, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 28, 167, 45, 23, 210, 106], OperandSize::Dword)
}

fn pabsb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 28, 248], OperandSize::Qword)
}

fn pabsb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSB, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexed(RSI, RBX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 28, 28, 94], OperandSize::Qword)
}

