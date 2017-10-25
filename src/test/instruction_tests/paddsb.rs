use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn paddsb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDSB, operand1: Some(Direct(MM0)), operand2: Some(Direct(MM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 236, 199], OperandSize::Dword)
}

fn paddsb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDSB, operand1: Some(Direct(MM4)), operand2: Some(IndirectScaledDisplaced(EDX, Eight, 341256381, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 236, 36, 213, 189, 40, 87, 20], OperandSize::Dword)
}

fn paddsb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDSB, operand1: Some(Direct(MM3)), operand2: Some(Direct(MM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 236, 216], OperandSize::Qword)
}

fn paddsb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDSB, operand1: Some(Direct(MM5)), operand2: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 236, 47], OperandSize::Qword)
}

fn paddsb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDSB, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 236, 233], OperandSize::Dword)
}

fn paddsb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDSB, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexed(EAX, ESI, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 236, 20, 240], OperandSize::Dword)
}

fn paddsb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDSB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 236, 217], OperandSize::Qword)
}

fn paddsb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDSB, operand1: Some(Direct(XMM7)), operand2: Some(IndirectDisplaced(RSI, 2132054931, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 236, 190, 147, 147, 20, 127], OperandSize::Qword)
}

