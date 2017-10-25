use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vaddss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 88, 236], OperandSize::Dword)
}

fn vaddss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(ECX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 226, 88, 17], OperandSize::Dword)
}

fn vaddss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 88, 197], OperandSize::Qword)
}

fn vaddss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectDisplaced(RDX, 2033258688, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 234, 88, 178, 192, 16, 49, 121], OperandSize::Qword)
}

fn vaddss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 70, 253, 88, 226], OperandSize::Dword)
}

fn vaddss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(EBX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 118, 142, 88, 35], OperandSize::Dword)
}

fn vaddss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSS, operand1: Some(Direct(XMM12)), operand2: Some(Direct(XMM20)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 113, 94, 210, 88, 225], OperandSize::Qword)
}

fn vaddss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSS, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM12)), operand3: Some(IndirectScaledIndexed(RSI, RSI, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 97, 30, 138, 88, 20, 246], OperandSize::Qword)
}

