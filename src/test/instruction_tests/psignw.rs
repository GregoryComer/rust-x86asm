use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn psignw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGNW, operand1: Some(Direct(MM6)), operand2: Some(Direct(MM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 9, 240], OperandSize::Dword)
}

fn psignw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGNW, operand1: Some(Direct(MM7)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, EBX, Eight, 1431613492, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 9, 188, 218, 52, 176, 84, 85], OperandSize::Dword)
}

fn psignw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGNW, operand1: Some(Direct(MM5)), operand2: Some(Direct(MM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 9, 232], OperandSize::Qword)
}

fn psignw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGNW, operand1: Some(Direct(MM7)), operand2: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 9, 57], OperandSize::Qword)
}

fn psignw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGNW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 9, 192], OperandSize::Dword)
}

fn psignw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGNW, operand1: Some(Direct(XMM0)), operand2: Some(IndirectDisplaced(ESI, 120657079, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 9, 134, 183, 20, 49, 7], OperandSize::Dword)
}

fn psignw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGNW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 9, 254], OperandSize::Qword)
}

fn psignw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGNW, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexed(RDX, RDX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 9, 60, 210], OperandSize::Qword)
}

