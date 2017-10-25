use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vfnmadd132ss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132SS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 157, 194], OperandSize::Dword)
}

fn vfnmadd132ss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132SS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(EBX, EBX, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 157, 20, 219], OperandSize::Dword)
}

fn vfnmadd132ss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132SS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 157, 192], OperandSize::Qword)
}

fn vfnmadd132ss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132SS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(RDI, RSI, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 157, 52, 119], OperandSize::Qword)
}

fn vfnmadd132ss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132SS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 77, 252, 157, 235], OperandSize::Dword)
}

fn vfnmadd132ss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132SS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledDisplaced(EDX, Two, 345057273, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 77, 138, 157, 4, 85, 249, 39, 145, 20], OperandSize::Dword)
}

fn vfnmadd132ss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132SS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 109, 220, 157, 222], OperandSize::Qword)
}

fn vfnmadd132ss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132SS, operand1: Some(Direct(XMM18)), operand2: Some(Direct(XMM29)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RDI, Two, 303753399, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 226, 21, 132, 157, 148, 122, 183, 232, 26, 18], OperandSize::Qword)
}

