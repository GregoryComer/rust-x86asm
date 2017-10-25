use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn por_1() {
    run_test(&Instruction { mnemonic: Mnemonic::POR, operand1: Some(Direct(MM0)), operand2: Some(Direct(MM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 235, 192], OperandSize::Dword)
}

fn por_2() {
    run_test(&Instruction { mnemonic: Mnemonic::POR, operand1: Some(Direct(MM7)), operand2: Some(IndirectScaledIndexed(ECX, EBX, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 235, 60, 89], OperandSize::Dword)
}

fn por_3() {
    run_test(&Instruction { mnemonic: Mnemonic::POR, operand1: Some(Direct(MM7)), operand2: Some(Direct(MM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 235, 248], OperandSize::Qword)
}

fn por_4() {
    run_test(&Instruction { mnemonic: Mnemonic::POR, operand1: Some(Direct(MM6)), operand2: Some(IndirectScaledIndexed(RCX, RBX, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 235, 52, 153], OperandSize::Qword)
}

fn por_5() {
    run_test(&Instruction { mnemonic: Mnemonic::POR, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 235, 247], OperandSize::Dword)
}

fn por_6() {
    run_test(&Instruction { mnemonic: Mnemonic::POR, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledDisplaced(EBX, Eight, 282229948, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 235, 52, 221, 188, 124, 210, 16], OperandSize::Dword)
}

fn por_7() {
    run_test(&Instruction { mnemonic: Mnemonic::POR, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 235, 206], OperandSize::Qword)
}

fn por_8() {
    run_test(&Instruction { mnemonic: Mnemonic::POR, operand1: Some(Direct(XMM6)), operand2: Some(IndirectDisplaced(RDI, 2106695575, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 235, 183, 151, 159, 145, 125], OperandSize::Qword)
}

