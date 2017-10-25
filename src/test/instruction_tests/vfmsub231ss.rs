use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vfmsub231ss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231SS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 187, 217], OperandSize::Dword)
}

fn vfmsub231ss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231SS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, ESI, Eight, 849967021, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 187, 180, 243, 173, 119, 169, 50], OperandSize::Dword)
}

fn vfmsub231ss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231SS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 187, 207], OperandSize::Qword)
}

fn vfmsub231ss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231SS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(RBX, 1981805369, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 187, 155, 57, 243, 31, 118], OperandSize::Qword)
}

fn vfmsub231ss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231SS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 77, 223, 187, 226], OperandSize::Dword)
}

fn vfmsub231ss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231SS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EDX, Four, 1386446332, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 101, 142, 187, 172, 150, 252, 125, 163, 82], OperandSize::Dword)
}

fn vfmsub231ss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231SS, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM12)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 98, 29, 249, 187, 215], OperandSize::Qword)
}

fn vfmsub231ss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231SS, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM25)), operand3: Some(IndirectScaledIndexed(RDX, RSI, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 98, 53, 132, 187, 12, 242], OperandSize::Qword)
}

