use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn pinsrd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PINSRD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(EBX)), operand3: Some(Literal8(21)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 34, 251, 21], OperandSize::Dword)
}

fn pinsrd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PINSRD, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, ESI, Eight, 516874095, Some(OperandSize::Dword), None)), operand3: Some(Literal8(30)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 34, 140, 241, 111, 223, 206, 30, 30], OperandSize::Dword)
}

fn pinsrd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PINSRD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(ESP)), operand3: Some(Literal8(44)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 34, 220, 44], OperandSize::Qword)
}

fn pinsrd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PINSRD, operand1: Some(Direct(XMM4)), operand2: Some(Indirect(RBX, Some(OperandSize::Dword), None)), operand3: Some(Literal8(100)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 34, 35, 100], OperandSize::Qword)
}

