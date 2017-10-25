use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vrsqrtps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRTPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 82, 247], OperandSize::Dword)
}

fn vrsqrtps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRTPS, operand1: Some(Direct(XMM1)), operand2: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 82, 15], OperandSize::Dword)
}

fn vrsqrtps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRTPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 82, 216], OperandSize::Qword)
}

fn vrsqrtps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRTPS, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(RDX, Two, 1728945695, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 82, 36, 85, 31, 158, 13, 103], OperandSize::Qword)
}

fn vrsqrtps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRTPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 82, 197], OperandSize::Dword)
}

fn vrsqrtps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRTPS, operand1: Some(Direct(YMM4)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, EAX, Four, 1313887329, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 82, 164, 129, 97, 84, 80, 78], OperandSize::Dword)
}

fn vrsqrtps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRTPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 82, 207], OperandSize::Qword)
}

fn vrsqrtps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRTPS, operand1: Some(Direct(YMM4)), operand2: Some(IndirectDisplaced(RAX, 1832224519, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 82, 160, 7, 135, 53, 109], OperandSize::Qword)
}

