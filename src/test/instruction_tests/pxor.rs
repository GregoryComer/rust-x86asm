use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn pxor_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PXOR, operand1: Some(Direct(MM3)), operand2: Some(Direct(MM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 239, 221], OperandSize::Dword)
}

fn pxor_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PXOR, operand1: Some(Direct(MM5)), operand2: Some(IndirectDisplaced(EDX, 1213312672, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 239, 170, 160, 174, 81, 72], OperandSize::Dword)
}

fn pxor_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PXOR, operand1: Some(Direct(MM0)), operand2: Some(Direct(MM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 239, 195], OperandSize::Qword)
}

fn pxor_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PXOR, operand1: Some(Direct(MM0)), operand2: Some(IndirectDisplaced(RDI, 1407035964, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 239, 135, 60, 170, 221, 83], OperandSize::Qword)
}

fn pxor_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PXOR, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 239, 208], OperandSize::Dword)
}

fn pxor_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PXOR, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, EAX, Four, 2034803238, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 239, 172, 131, 38, 162, 72, 121], OperandSize::Dword)
}

fn pxor_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PXOR, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 239, 239], OperandSize::Qword)
}

fn pxor_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PXOR, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(RCX, Eight, 759138735, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 239, 36, 205, 175, 137, 63, 45], OperandSize::Qword)
}

