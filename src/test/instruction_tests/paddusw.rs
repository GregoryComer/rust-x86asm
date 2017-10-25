use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn paddusw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDUSW, operand1: Some(Direct(MM2)), operand2: Some(Direct(MM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 221, 215], OperandSize::Dword)
}

fn paddusw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDUSW, operand1: Some(Direct(MM4)), operand2: Some(IndirectScaledIndexed(EDX, ECX, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 221, 36, 74], OperandSize::Dword)
}

fn paddusw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDUSW, operand1: Some(Direct(MM6)), operand2: Some(Direct(MM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 221, 240], OperandSize::Qword)
}

fn paddusw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDUSW, operand1: Some(Direct(MM0)), operand2: Some(IndirectScaledDisplaced(RSI, Eight, 1908134084, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 221, 4, 245, 196, 208, 187, 113], OperandSize::Qword)
}

fn paddusw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDUSW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 221, 201], OperandSize::Dword)
}

fn paddusw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDUSW, operand1: Some(Direct(XMM5)), operand2: Some(IndirectDisplaced(EDX, 1665767062, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 221, 170, 150, 150, 73, 99], OperandSize::Dword)
}

fn paddusw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDUSW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 221, 228], OperandSize::Qword)
}

fn paddusw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDUSW, operand1: Some(Direct(XMM2)), operand2: Some(IndirectDisplaced(RDX, 1463422630, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 221, 146, 166, 14, 58, 87], OperandSize::Qword)
}

