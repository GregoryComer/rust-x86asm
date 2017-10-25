use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vmovhlps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVHLPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 240, 18, 236], OperandSize::Dword)
}

fn vmovhlps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVHLPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 232, 18, 244], OperandSize::Qword)
}

fn vmovhlps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVHLPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 192, 18, 255], OperandSize::Dword)
}

fn vmovhlps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVHLPS, operand1: Some(Direct(XMM29)), operand2: Some(Direct(XMM28)), operand3: Some(Direct(XMM13)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 65, 28, 0, 18, 237], OperandSize::Qword)
}

