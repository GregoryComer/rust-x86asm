use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vmaxss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXSS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 194, 95, 198], OperandSize::Dword)
}

fn vmaxss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXSS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(EDI, ECX, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 210, 95, 36, 207], OperandSize::Dword)
}

fn vmaxss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXSS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 95, 218], OperandSize::Qword)
}

fn vmaxss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXSS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(RBX, Four, 360101629, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 226, 95, 20, 157, 253, 182, 118, 21], OperandSize::Qword)
}

fn vmaxss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXSS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 126, 155, 95, 242], OperandSize::Dword)
}

fn vmaxss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXSS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(EDI, ESI, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 78, 139, 95, 44, 119], OperandSize::Dword)
}

fn vmaxss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXSS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM17)), operand3: Some(Direct(XMM10)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K6), broadcast: None }, &[98, 209, 118, 150, 95, 250], OperandSize::Qword)
}

fn vmaxss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXSS, operand1: Some(Direct(XMM28)), operand2: Some(Direct(XMM21)), operand3: Some(IndirectDisplaced(RSI, 828957709, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 97, 86, 135, 95, 166, 13, 228, 104, 49], OperandSize::Qword)
}

