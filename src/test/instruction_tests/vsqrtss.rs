use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vsqrtss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTSS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 218, 81, 230], OperandSize::Dword)
}

fn vsqrtss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTSS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(EBX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 234, 81, 27], OperandSize::Dword)
}

fn vsqrtss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTSS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 202, 81, 246], OperandSize::Qword)
}

fn vsqrtss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTSS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(RBX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 242, 81, 19], OperandSize::Qword)
}

fn vsqrtss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTSS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 118, 185, 81, 242], OperandSize::Dword)
}

fn vsqrtss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTSS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(EBX, 1997036612, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 78, 142, 81, 179, 68, 92, 8, 119], OperandSize::Dword)
}

fn vsqrtss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTSS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM17)), operand3: Some(Direct(XMM10)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 209, 118, 149, 81, 194], OperandSize::Qword)
}

fn vsqrtss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTSS, operand1: Some(Direct(XMM20)), operand2: Some(Direct(XMM9)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RSI, Two, 2114798414, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 225, 54, 139, 81, 164, 114, 78, 67, 13, 126], OperandSize::Qword)
}

