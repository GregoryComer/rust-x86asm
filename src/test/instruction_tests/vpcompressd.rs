use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpcompressd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 142, 139, 211], OperandSize::Dword)
}

fn vpcompressd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSD, operand1: Some(IndirectDisplaced(ECX, 1126373660, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 125, 8, 139, 161, 28, 25, 35, 67], OperandSize::Dword)
}

fn vpcompressd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSD, operand1: Some(Direct(XMM13)), operand2: Some(Direct(XMM15)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 82, 125, 141, 139, 253], OperandSize::Qword)
}

fn vpcompressd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSD, operand1: Some(IndirectScaledIndexed(RDI, RBX, Eight, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM20)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 226, 125, 8, 139, 36, 223], OperandSize::Qword)
}

fn vpcompressd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 169, 139, 214], OperandSize::Dword)
}

fn vpcompressd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSD, operand1: Some(IndirectScaledIndexedDisplaced(ECX, ESI, Four, 1418872773, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 125, 40, 139, 132, 177, 197, 71, 146, 84], OperandSize::Dword)
}

fn vpcompressd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM27)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 98, 125, 172, 139, 221], OperandSize::Qword)
}

fn vpcompressd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSD, operand1: Some(IndirectScaledIndexedDisplaced(RDX, RSI, Two, 179933283, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM17)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 226, 125, 40, 139, 140, 114, 99, 144, 185, 10], OperandSize::Qword)
}

fn vpcompressd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 206, 139, 230], OperandSize::Dword)
}

fn vpcompressd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSD, operand1: Some(IndirectScaledDisplaced(EDX, Two, 1697490412, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 125, 72, 139, 60, 85, 236, 165, 45, 101], OperandSize::Dword)
}

fn vpcompressd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSD, operand1: Some(Direct(ZMM12)), operand2: Some(Direct(ZMM10)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 82, 125, 206, 139, 212], OperandSize::Qword)
}

fn vpcompressd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSD, operand1: Some(Indirect(RSI, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 125, 72, 139, 30], OperandSize::Qword)
}

