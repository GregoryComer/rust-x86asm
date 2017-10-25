use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vmovmskps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVMSKPS, operand1: Some(Direct(EDI)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 80, 254], OperandSize::Dword)
}

fn vmovmskps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVMSKPS, operand1: Some(Direct(RDI)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 80, 248], OperandSize::Qword)
}

fn vmovmskps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVMSKPS, operand1: Some(Direct(ESI)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 80, 242], OperandSize::Dword)
}

fn vmovmskps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVMSKPS, operand1: Some(Direct(RSI)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 80, 243], OperandSize::Qword)
}

