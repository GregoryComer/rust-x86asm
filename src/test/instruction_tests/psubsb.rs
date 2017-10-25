use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn psubsb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBSB, operand1: Some(Direct(MM7)), operand2: Some(Direct(MM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 232, 255], OperandSize::Dword)
}

fn psubsb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBSB, operand1: Some(Direct(MM4)), operand2: Some(Indirect(ESI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 232, 38], OperandSize::Dword)
}

fn psubsb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBSB, operand1: Some(Direct(MM5)), operand2: Some(Direct(MM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 232, 233], OperandSize::Qword)
}

fn psubsb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBSB, operand1: Some(Direct(MM7)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RDX, Four, 820335708, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 232, 188, 144, 92, 84, 229, 48], OperandSize::Qword)
}

fn psubsb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBSB, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 232, 232], OperandSize::Dword)
}

fn psubsb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBSB, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledDisplaced(ESI, Two, 701009715, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 232, 4, 117, 51, 143, 200, 41], OperandSize::Dword)
}

fn psubsb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBSB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 232, 223], OperandSize::Qword)
}

fn psubsb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBSB, operand1: Some(Direct(XMM2)), operand2: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 232, 17], OperandSize::Qword)
}

