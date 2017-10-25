use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vmovmskpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVMSKPD, operand1: Some(Direct(ESI)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 80, 241], OperandSize::Dword)
}

fn vmovmskpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVMSKPD, operand1: Some(Direct(RBX)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 80, 218], OperandSize::Qword)
}

fn vmovmskpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVMSKPD, operand1: Some(Direct(ECX)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 80, 207], OperandSize::Dword)
}

fn vmovmskpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVMSKPD, operand1: Some(Direct(RSP)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 80, 231], OperandSize::Qword)
}

