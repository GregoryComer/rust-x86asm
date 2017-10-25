use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vrsqrtss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRTSS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 210, 82, 221], OperandSize::Dword)
}

fn vrsqrtss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRTSS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(EDX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 242, 82, 58], OperandSize::Dword)
}

fn vrsqrtss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRTSS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 82, 233], OperandSize::Qword)
}

fn vrsqrtss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRTSS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectDisplaced(RDX, 762789945, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 82, 138, 57, 64, 119, 45], OperandSize::Qword)
}

