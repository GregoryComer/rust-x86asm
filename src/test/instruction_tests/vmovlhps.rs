use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vmovlhps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVLHPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 192, 22, 255], OperandSize::Dword)
}

fn vmovlhps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVLHPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 200, 22, 237], OperandSize::Qword)
}

fn vmovlhps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVLHPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 232, 22, 197], OperandSize::Dword)
}

fn vmovlhps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVLHPS, operand1: Some(Direct(XMM9)), operand2: Some(Direct(XMM29)), operand3: Some(Direct(XMM17)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 49, 20, 0, 22, 201], OperandSize::Qword)
}

