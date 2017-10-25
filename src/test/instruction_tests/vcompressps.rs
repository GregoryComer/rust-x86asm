use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vcompressps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 142, 138, 222], OperandSize::Dword)
}

fn vcompressps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPS, operand1: Some(IndirectScaledIndexedDisplaced(EAX, EBX, Four, 72822324, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 125, 8, 138, 164, 152, 52, 46, 87, 4], OperandSize::Dword)
}

fn vcompressps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPS, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 178, 125, 140, 138, 249], OperandSize::Qword)
}

fn vcompressps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPS, operand1: Some(IndirectDisplaced(RCX, 1570940812, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM24)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 98, 125, 8, 138, 129, 140, 167, 162, 93], OperandSize::Qword)
}

fn vcompressps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 170, 138, 210], OperandSize::Dword)
}

fn vcompressps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPS, operand1: Some(IndirectScaledIndexedDisplaced(EBX, EDI, Eight, 1461039954, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 125, 40, 138, 172, 251, 82, 179, 21, 87], OperandSize::Dword)
}

fn vcompressps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPS, operand1: Some(Direct(YMM30)), operand2: Some(Direct(YMM13)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 18, 125, 171, 138, 238], OperandSize::Qword)
}

fn vcompressps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPS, operand1: Some(IndirectScaledIndexedDisplaced(RAX, RAX, Two, 1843706159, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM17)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 226, 125, 40, 138, 140, 64, 47, 185, 228, 109], OperandSize::Qword)
}

fn vcompressps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPS, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 204, 138, 251], OperandSize::Dword)
}

fn vcompressps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPS, operand1: Some(IndirectScaledIndexed(EDX, ESI, Four, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 125, 72, 138, 20, 178], OperandSize::Dword)
}

fn vcompressps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPS, operand1: Some(Direct(ZMM12)), operand2: Some(Direct(ZMM12)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 82, 125, 207, 138, 228], OperandSize::Qword)
}

fn vcompressps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPS, operand1: Some(IndirectScaledDisplaced(RDX, Four, 1765368513, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 125, 72, 138, 28, 149, 193, 98, 57, 105], OperandSize::Qword)
}

