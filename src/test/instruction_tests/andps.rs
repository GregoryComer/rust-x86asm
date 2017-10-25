use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn andps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::ANDPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 84, 226], OperandSize::Dword)
}

fn andps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::ANDPS, operand1: Some(Direct(XMM5)), operand2: Some(IndirectDisplaced(ESI, 340451203, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 84, 174, 131, 223, 74, 20], OperandSize::Dword)
}

fn andps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::ANDPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 84, 200], OperandSize::Qword)
}

fn andps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::ANDPS, operand1: Some(Direct(XMM0)), operand2: Some(IndirectDisplaced(RDI, 490367348, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 84, 135, 116, 105, 58, 29], OperandSize::Qword)
}

