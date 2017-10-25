use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn psignb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGNB, operand1: Some(Direct(MM4)), operand2: Some(Direct(MM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 8, 231], OperandSize::Dword)
}

fn psignb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGNB, operand1: Some(Direct(MM5)), operand2: Some(Indirect(EAX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 8, 40], OperandSize::Dword)
}

fn psignb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGNB, operand1: Some(Direct(MM5)), operand2: Some(Direct(MM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 8, 239], OperandSize::Qword)
}

fn psignb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGNB, operand1: Some(Direct(MM3)), operand2: Some(Indirect(RSI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 8, 30], OperandSize::Qword)
}

fn psignb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGNB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 8, 198], OperandSize::Dword)
}

fn psignb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGNB, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EAX, Two, 91267760, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 8, 172, 64, 176, 162, 112, 5], OperandSize::Dword)
}

fn psignb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGNB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 8, 254], OperandSize::Qword)
}

fn psignb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGNB, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexed(RBX, RBX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 8, 12, 219], OperandSize::Qword)
}

