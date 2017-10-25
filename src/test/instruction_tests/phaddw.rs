use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn phaddw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDW, operand1: Some(Direct(MM7)), operand2: Some(Direct(MM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 1, 250], OperandSize::Dword)
}

fn phaddw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDW, operand1: Some(Direct(MM0)), operand2: Some(IndirectScaledIndexed(EAX, EBX, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 1, 4, 88], OperandSize::Dword)
}

fn phaddw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDW, operand1: Some(Direct(MM5)), operand2: Some(Direct(MM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 1, 237], OperandSize::Qword)
}

fn phaddw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDW, operand1: Some(Direct(MM6)), operand2: Some(IndirectScaledDisplaced(RBX, Eight, 1923503008, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 1, 52, 221, 160, 83, 166, 114], OperandSize::Qword)
}

fn phaddw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 1, 243], OperandSize::Dword)
}

fn phaddw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDW, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledDisplaced(EBX, Four, 1742689237, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 1, 44, 157, 213, 83, 223, 103], OperandSize::Dword)
}

fn phaddw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 1, 231], OperandSize::Qword)
}

fn phaddw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDW, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexed(RSI, RBX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 1, 4, 222], OperandSize::Qword)
}

