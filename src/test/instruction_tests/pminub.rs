use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn pminub_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINUB, operand1: Some(Direct(MM7)), operand2: Some(Direct(MM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 218, 249], OperandSize::Dword)
}

fn pminub_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINUB, operand1: Some(Direct(MM3)), operand2: Some(IndirectScaledDisplaced(EDI, Two, 286292411, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 218, 28, 125, 187, 121, 16, 17], OperandSize::Dword)
}

fn pminub_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINUB, operand1: Some(Direct(MM1)), operand2: Some(Direct(MM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 218, 206], OperandSize::Qword)
}

fn pminub_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINUB, operand1: Some(Direct(MM6)), operand2: Some(IndirectScaledDisplaced(RSI, Eight, 1492764750, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 218, 52, 245, 78, 200, 249, 88], OperandSize::Qword)
}

fn pminub_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINUB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 218, 201], OperandSize::Dword)
}

fn pminub_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINUB, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, ESI, Four, 1518706079, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 218, 188, 179, 159, 157, 133, 90], OperandSize::Dword)
}

fn pminub_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINUB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 218, 241], OperandSize::Qword)
}

fn pminub_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINUB, operand1: Some(Direct(XMM2)), operand2: Some(IndirectDisplaced(RBX, 819510012, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 218, 147, 252, 186, 216, 48], OperandSize::Qword)
}

